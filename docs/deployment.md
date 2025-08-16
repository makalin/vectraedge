# VectraEdge Deployment Guide

This guide covers deploying VectraEdge in production environments, including Docker, Kubernetes, and cloud platforms.

## 🐳 Docker Deployment

### Production Docker Image

#### Build Production Image
```bash
# Build optimized production image
make prod

# Or manually
docker build -t vectraedge/vectra:latest \
  --build-arg BUILD_TYPE=release \
  --build-arg FEATURES=production .
```

#### Run Production Container
```bash
# Basic run
docker run -d \
  --name vectra \
  -p 8080:8080 \
  -p 6432:6432 \
  -v /data/vectra:/app/data \
  -e VECTRA_LOG_LEVEL=warn \
  vectraedge/vectra:latest

# With custom configuration
docker run -d \
  --name vectra \
  -p 8080:8080 \
  -v /data/vectra:/app/data \
  -v /config:/app/config \
  -e VECTRA_CONFIG_PATH=/app/config/vectra.toml \
  vectraedge/vectra:latest
```

#### Docker Compose Production
```yaml
# docker-compose.prod.yml
version: '3.8'

services:
  vectra:
    image: vectraedge/vectra:latest
    container_name: vectra
    ports:
      - "8080:8080"
      - "6432:6432"
    volumes:
      - vectra_data:/app/data
      - ./config:/app/config
    environment:
      - VECTRA_LOG_LEVEL=warn
      - VECTRA_CONFIG_PATH=/app/config/vectra.toml
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
    deploy:
      resources:
        limits:
          memory: 4G
          cpus: '2.0'
        reservations:
          memory: 2G
          cpus: '1.0'

  redpanda:
    image: vectorized/redpanda:latest
    container_name: redpanda
    ports:
      - "8081:8081"
      - "9092:9092"
    volumes:
      - redpanda_data:/var/lib/redpanda/data
    command: >
      redpanda start
      --smp 1
      --memory 1G
      --reserve-memory 0M
      --overprovisioned
      --node-id 0
      --check=false
      --pandaproxy-addr 0.0.0.0:8081
      --advertise-pandaproxy-addr localhost:8081
      --kafka-addr 0.0.0.0:9092
      --advertise-kafka-addr localhost:9092
    restart: unless-stopped

volumes:
  vectra_data:
    driver: local
  redpanda_data:
    driver: local
```

### Production Configuration
```toml
# config/vectra.prod.toml
[server]
host = "0.0.0.0"
port = 8080
workers = 8
max_connections = 2000
request_timeout = 60
max_request_size = "50MB"

[security]
jwt_secret = "${VECTRA_JWT_SECRET}"
jwt_expiry = "24h"
cors_origins = ["https://yourdomain.com"]
rate_limit = 5000

[storage]
data_dir = "/app/data"
max_memory_mb = 4096
compression = true
backup_enabled = true
backup_interval = "1h"
backup_retention = "7d"

[vector_search]
dimension = 384
m = 16
ef_construction = 200
ef = 50
distance_metric = "cosine"

[ai]
ollama_url = "http://ollama:11434"
embedding_model = "text-embedding-ada-002"
text_model = "llama2"
max_tokens = 2048
temperature = 0.7

[logging]
level = "warn"
format = "json"
output = "stdout"

[monitoring]
metrics_enabled = true
prometheus_endpoint = "/metrics"
health_check_interval = "30s"
```

## ☸️ Kubernetes Deployment

### Basic Deployment
```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: vectra-edge
  labels:
    app: vectra-edge
spec:
  replicas: 3
  selector:
    matchLabels:
      app: vectra-edge
  template:
    metadata:
      labels:
        app: vectra-edge
    spec:
      containers:
      - name: vectra
        image: vectraedge/vectra:latest
        ports:
        - containerPort: 8080
          name: http
        - containerPort: 6432
          name: postgres
        env:
        - name: VECTRA_HOST
          value: "0.0.0.0"
        - name: VECTRA_PORT
          value: "8080"
        - name: VECTRA_LOG_LEVEL
          value: "warn"
        - name: VECTRA_CONFIG_PATH
          value: "/app/config/vectra.toml"
        volumeMounts:
        - name: config
          mountPath: /app/config
        - name: data
          mountPath: /app/data
        resources:
          requests:
            memory: "2Gi"
            cpu: "1"
          limits:
            memory: "4Gi"
            cpu: "2"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: config
        configMap:
          name: vectra-config
      - name: data
        persistentVolumeClaim:
          claimName: vectra-data
---
apiVersion: v1
kind: Service
metadata:
  name: vectra-service
spec:
  selector:
    app: vectra-edge
  ports:
  - name: http
    port: 8080
    targetPort: 8080
  - name: postgres
    port: 6432
    targetPort: 6432
  type: ClusterIP
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: vectra-config
data:
  vectra.toml: |
    [server]
    host = "0.0.0.0"
    port = 8080
    workers = 4
    
    [storage]
    data_dir = "/app/data"
    max_memory_mb = 2048
    
    [vector_search]
    dimension = 384
    m = 16
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: vectra-data
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 100Gi
  storageClassName: fast-ssd
```

### Ingress Configuration
```yaml
# k8s/ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: vectra-ingress
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  tls:
  - hosts:
    - vectra.yourdomain.com
    secretName: vectra-tls
  rules:
  - host: vectra.yourdomain.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: vectra-service
            port:
              number: 8080
```

### Horizontal Pod Autoscaler
```yaml
# k8s/hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: vectra-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: vectra-edge
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

## ☁️ Cloud Platform Deployment

### AWS ECS/Fargate
```yaml
# aws/task-definition.json
{
  "family": "vectra-edge",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "2048",
  "memory": "4096",
  "executionRoleArn": "arn:aws:iam::ACCOUNT:role/ecsTaskExecutionRole",
  "taskRoleArn": "arn:aws:iam::ACCOUNT:role/vectra-task-role",
  "containerDefinitions": [
    {
      "name": "vectra",
      "image": "vectraedge/vectra:latest",
      "portMappings": [
        {
          "containerPort": 8080,
          "protocol": "tcp"
        }
      ],
      "environment": [
        {
          "name": "VECTRA_LOG_LEVEL",
          "value": "warn"
        },
        {
          "name": "VECTRA_CONFIG_PATH",
          "value": "/app/config/vectra.toml"
        }
      ],
      "mountPoints": [
        {
          "sourceVolume": "config",
          "containerPath": "/app/config",
          "readOnly": true
        }
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/vectra-edge",
          "awslogs-region": "us-west-2",
          "awslogs-stream-prefix": "ecs"
        }
      },
      "healthCheck": {
        "command": ["CMD-SHELL", "curl -f http://localhost:8080/health || exit 1"],
        "interval": 30,
        "timeout": 5,
        "retries": 3,
        "startPeriod": 60
      }
    }
  ],
  "volumes": [
    {
      "name": "config",
      "configMap": {
        "name": "vectra-config"
      }
    }
  ]
}
```

### Google Cloud Run
```yaml
# gcp/cloud-run.yaml
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: vectra-edge
spec:
  template:
    metadata:
      annotations:
        autoscaling.knative.dev/minScale: "1"
        autoscaling.knative.dev/maxScale: "10"
    spec:
      containerConcurrency: 100
      timeoutSeconds: 300
      containers:
      - image: gcr.io/PROJECT/vectra:latest
        ports:
        - containerPort: 8080
        env:
        - name: VECTRA_HOST
          value: "0.0.0.0"
        - name: VECTRA_PORT
          value: "8080"
        - name: VECTRA_LOG_LEVEL
          value: "warn"
        resources:
          limits:
            cpu: "2000m"
            memory: "4Gi"
          requests:
            cpu: "1000m"
            memory: "2Gi"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
```

## 🔒 Security Configuration

### SSL/TLS Setup
```bash
# Generate self-signed certificate (development)
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes

# Let's Encrypt (production)
certbot certonly --standalone -d vectra.yourdomain.com

# Configure VectraEdge with SSL
```

```toml
# config/vectra.ssl.toml
[server]
host = "0.0.0.0"
port = 8080
ssl_enabled = true
ssl_cert_path = "/etc/ssl/certs/vectra.crt"
ssl_key_path = "/etc/ssl/private/vectra.key"
ssl_ca_path = "/etc/ssl/certs/ca-bundle.crt"
```

### Authentication & Authorization
```toml
# config/vectra.auth.toml
[security]
jwt_secret = "${VECTRA_JWT_SECRET}"
jwt_expiry = "24h"
cors_origins = ["https://yourdomain.com"]
rate_limit = 5000

[auth]
enabled = true
providers = ["jwt", "oauth2"]

[oauth2]
client_id = "${OAUTH_CLIENT_ID}"
client_secret = "${OAUTH_CLIENT_SECRET}"
redirect_uri = "https://vectra.yourdomain.com/auth/callback"
scopes = ["openid", "profile", "email"]

[ldap]
enabled = false
url = "ldap://ldap.yourdomain.com:389"
base_dn = "dc=yourdomain,dc=com"
bind_dn = "cn=vectra,dc=yourdomain,dc=com"
bind_password = "${LDAP_BIND_PASSWORD}"
```

## 📊 Monitoring & Observability

### Prometheus Metrics
```yaml
# k8s/monitoring.yaml
apiVersion: v1
kind: ServiceMonitor
metadata:
  name: vectra-monitor
  labels:
    release: prometheus
spec:
  selector:
    matchLabels:
      app: vectra-edge
  endpoints:
  - port: http
    path: /metrics
    interval: 30s
---
apiVersion: v1
kind: PrometheusRule
metadata:
  name: vectra-alerts
  labels:
    release: prometheus
spec:
  groups:
  - name: vectra
    rules:
    - alert: VectraHighMemoryUsage
      expr: vectra_memory_usage_bytes / vectra_memory_limit_bytes > 0.8
      for: 5m
      labels:
        severity: warning
      annotations:
        summary: "VectraEdge memory usage is high"
        description: "Memory usage is {{ $value | humanizePercentage }}"
    
    - alert: VectraHighQueryLatency
      expr: histogram_quantile(0.95, vectra_query_duration_seconds) > 1
      for: 5m
      labels:
        severity: warning
      annotations:
        summary: "VectraEdge query latency is high"
        description: "95th percentile query latency is {{ $value }}s"
```

### Grafana Dashboard
```json
// grafana/dashboard.json
{
  "dashboard": {
    "title": "VectraEdge Dashboard",
    "panels": [
      {
        "title": "Query Performance",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(vectra_queries_total[5m])",
            "legendFormat": "Queries/sec"
          }
        ]
      },
      {
        "title": "Memory Usage",
        "type": "gauge",
        "targets": [
          {
            "expr": "vectra_memory_usage_bytes / vectra_memory_limit_bytes * 100",
            "legendFormat": "Memory %"
          }
        ]
      },
      {
        "title": "Vector Search Latency",
        "type": "heatmap",
        "targets": [
          {
            "expr": "rate(vectra_vector_search_duration_seconds_bucket[5m])",
            "legendFormat": "Latency"
          }
        ]
      }
    ]
  }
}
```

## 🚀 Performance Tuning

### System Tuning
```bash
# Increase file descriptor limits
echo "* soft nofile 65536" >> /etc/security/limits.conf
echo "* hard nofile 65536" >> /etc/security/limits.conf

# Optimize kernel parameters
echo "vm.swappiness=1" >> /etc/sysctl.conf
echo "vm.max_map_count=262144" >> /etc/sysctl.conf
echo "net.core.somaxconn=65535" >> /etc/sysctl.conf

# Apply changes
sysctl -p
```

### VectraEdge Tuning
```toml
# config/vectra.performance.toml
[server]
workers = 16
max_connections = 10000
request_timeout = 120
max_request_size = "100MB"

[storage]
max_memory_mb = 8192
compression = true
compression_level = 6
backup_enabled = true
backup_compression = true

[vector_search]
dimension = 384
m = 32
ef_construction = 400
ef = 100
distance_metric = "cosine"
parallel_search = true
search_threads = 8

[cache]
enabled = true
max_size_mb = 1024
ttl_seconds = 3600
eviction_policy = "lru"

[query_optimizer]
enable_parallel_execution = true
max_parallel_workers = 8
enable_vectorization = true
enable_predicate_pushdown = true
```

## 🔄 Backup & Recovery

### Automated Backups
```bash
#!/bin/bash
# scripts/backup.sh

BACKUP_DIR="/backups/vectra"
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_NAME="vectra_backup_$DATE"

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Stop VectraEdge gracefully
docker exec vectra pkill -TERM vectra

# Wait for graceful shutdown
sleep 10

# Create backup
tar -czf "$BACKUP_DIR/$BACKUP_NAME.tar.gz" -C /data/vectra .

# Restart VectraEdge
docker start vectra

# Clean old backups (keep last 7 days)
find "$BACKUP_DIR" -name "vectra_backup_*.tar.gz" -mtime +7 -delete

# Upload to S3 (optional)
aws s3 cp "$BACKUP_DIR/$BACKUP_NAME.tar.gz" "s3://your-backup-bucket/vectra/"
```

### Recovery Process
```bash
#!/bin/bash
# scripts/restore.sh

BACKUP_FILE="$1"
RESTORE_DIR="/data/vectra"

if [ -z "$BACKUP_FILE" ]; then
    echo "Usage: $0 <backup_file>"
    exit 1
fi

# Stop VectraEdge
docker stop vectra

# Clean existing data
rm -rf "$RESTORE_DIR"/*

# Extract backup
tar -xzf "$BACKUP_FILE" -C "$RESTORE_DIR"

# Fix permissions
chown -R 1000:1000 "$RESTORE_DIR"

# Start VectraEdge
docker start vectra

echo "Recovery completed successfully"
```

## 📋 Deployment Checklist

### Pre-deployment
- [ ] Environment variables configured
- [ ] SSL certificates obtained
- [ ] Database backups scheduled
- [ ] Monitoring configured
- [ ] Load balancer configured
- [ ] DNS records updated

### Deployment
- [ ] Health checks passing
- [ ] Metrics collection working
- [ ] Log aggregation configured
- [ ] Backup system tested
- [ ] Performance benchmarks run
- [ ] Security scan completed

### Post-deployment
- [ ] Load testing completed
- [ ] Monitoring alerts configured
- [ ] Documentation updated
- [ ] Team training completed
- [ ] Support procedures documented
- [ ] Rollback plan tested

---

*For more deployment options and advanced configurations, check the [Configuration Guide](configuration.md) and [Troubleshooting](troubleshooting.md) sections.*
