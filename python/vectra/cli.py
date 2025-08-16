#!/usr/bin/env python3
"""
VectraEdge Python CLI

Command-line interface for the VectraEdge AI-Native OLAP Engine.
"""

import argparse
import asyncio
import json
import sys
from typing import Optional, Dict, Any

from .client import VectraClient


def main():
    """Main CLI entry point."""
    parser = argparse.ArgumentParser(
        description="VectraEdge - AI-Native OLAP Engine",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  vectra query "SELECT * FROM docs LIMIT 5"
  vectra search "machine learning" --limit 10
  vectra subscribe my_topic
  vectra create-table users "id INT, name TEXT, embedding VECTOR(384)"
  vectra interactive
        """
    )
    
    parser.add_argument(
        "--host", 
        default="127.0.0.1",
        help="VectraEdge server host (default: 127.0.0.1)"
    )
    parser.add_argument(
        "--port", 
        type=int, 
        default=8080,
        help="VectraEdge server port (default: 8080)"
    )
    
    subparsers = parser.add_subparsers(dest="command", help="Available commands")
    
    # Query command
    query_parser = subparsers.add_parser("query", help="Execute SQL query")
    query_parser.add_argument("sql", help="SQL query to execute")
    
    # Search command
    search_parser = subparsers.add_parser("search", help="Perform vector search")
    search_parser.add_argument("query", help="Query text for vector search")
    search_parser.add_argument("--limit", type=int, default=10, help="Maximum results (default: 10)")
    
    # Subscribe command
    subscribe_parser = subparsers.add_parser("subscribe", help="Subscribe to stream")
    subscribe_parser.add_argument("topic", help="Topic name to subscribe to")
    
    # Create table command
    create_table_parser = subparsers.add_parser("create-table", help="Create a table")
    create_table_parser.add_argument("table", help="Table name")
    create_table_parser.add_argument("schema", help="Table schema")
    
    # Insert command
    insert_parser = subparsers.add_parser("insert", help="Insert data into table")
    insert_parser.add_argument("table", help="Table name")
    insert_parser.add_argument("data", help="JSON data to insert")
    
    # Create index command
    create_index_parser = subparsers.add_parser("create-index", help="Create vector index")
    create_index_parser.add_argument("table", help="Table name")
    create_index_parser.add_argument("column", help="Column name")
    
    # List tables command
    subparsers.add_parser("list-tables", help="List all tables")
    
    # Table info command
    table_info_parser = subparsers.add_parser("table-info", help="Get table information")
    table_info_parser.add_argument("table", help="Table name")
    
    # Stats command
    subparsers.add_parser("stats", help="Get storage statistics")
    
    # Health command
    subparsers.add_parser("health", help="Health check")
    
    # Interactive command
    subparsers.add_parser("interactive", help="Start interactive mode")
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return 1
    
    try:
        client = VectraClient(host=args.host, port=args.port)
        
        if args.command == "query":
            result = client.execute_query(args.sql)
            print(json.dumps(result, indent=2))
            
        elif args.command == "search":
            result = client.vector_search(args.query, args.limit)
            print("Vector Search Results:")
            print(json.dumps(result, indent=2))
            
        elif args.command == "subscribe":
            subscription = client.subscribe_stream(args.topic)
            print("Stream Subscription Created:")
            print(f"  ID: {subscription.get_id()}")
            print(f"  Topic: {subscription.get_topic()}")
            print(f"  Status: {subscription.get_status()}")
            
        elif args.command == "create-table":
            client.create_table(args.table, args.schema)
            print(f"Table '{args.table}' created successfully")
            
        elif args.command == "insert":
            try:
                data = json.loads(args.data)
                client.insert_data(args.table, data)
                print(f"Data inserted into table '{args.table}' successfully")
            except json.JSONDecodeError:
                print("Error: Invalid JSON data", file=sys.stderr)
                return 1
                
        elif args.command == "create-index":
            index = client.create_vector_index(args.table, args.column)
            print(f"Vector index created on {args.table}.{args.column}")
            
        elif args.command == "list-tables":
            tables = client.list_tables()
            print("Tables:")
            for table in tables:
                print(f"  - {table}")
                
        elif args.command == "table-info":
            info = client.get_table_info(args.table)
            print(f"Table Info for '{args.table}':")
            print(json.dumps(info, indent=2))
            
        elif args.command == "stats":
            stats = client.get_stats()
            print("Storage Statistics:")
            print(json.dumps(stats, indent=2))
            
        elif args.command == "health":
            from . import health_check
            health = health_check()
            print("Health Check:")
            print(json.dumps(health, indent=2))
            
        elif args.command == "interactive":
            run_interactive_mode(client)
            
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        return 1
    
    return 0


def run_interactive_mode(client: VectraClient):
    """Run interactive mode."""
    print("VectraEdge Interactive Mode")
    print("Type 'help' for commands, 'quit' to exit")
    print()
    
    while True:
        try:
            command = input("vectra> ").strip()
            
            if not command:
                continue
                
            if command in ["quit", "exit"]:
                break
                
            if command == "help":
                show_interactive_help()
                continue
                
            # Try to execute as SQL query
            if command.lower().startswith(("select", "create", "insert", "update", "delete")):
                try:
                    result = client.execute_query(command)
                    print(json.dumps(result, indent=2))
                except Exception as e:
                    print(f"Error: {e}")
            else:
                print("Unknown command. Type 'help' for available commands.")
                
        except KeyboardInterrupt:
            print("\nExiting...")
            break
        except EOFError:
            break


def show_interactive_help():
    """Show interactive mode help."""
    print("Available commands:")
    print("  SQL queries: SELECT, CREATE, INSERT, UPDATE, DELETE")
    print("  help        - Show this help")
    print("  quit/exit   - Exit interactive mode")
    print()


if __name__ == "__main__":
    sys.exit(main())
