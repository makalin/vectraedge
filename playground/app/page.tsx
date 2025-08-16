'use client';

import { useState } from 'react';
import { 
  Database, 
  Search, 
  Zap, 
  Brain, 
  BarChart3, 
  Play,
  Code,
  Terminal,
  Settings,
  HelpCircle
} from 'lucide-react';
import { cn } from '@/lib/utils';

export default function PlaygroundPage() {
  const [activeTab, setActiveTab] = useState('sql');
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<any>(null);
  const [isLoading, setIsLoading] = useState(false);

  const tabs = [
    { id: 'sql', label: 'SQL Editor', icon: Database },
    { id: 'vector', label: 'Vector Search', icon: Search },
    { id: 'streaming', label: 'Streaming', icon: Zap },
    { id: 'ai', label: 'AI Models', icon: Brain },
    { id: 'analytics', label: 'Analytics', icon: BarChart3 },
  ];

  const handleExecute = async () => {
    if (!query.trim()) return;
    
    setIsLoading(true);
    try {
      // Mock API call - replace with actual VectraEdge API
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      setResults({
        rows: 1,
        data: [
          {
            result: "Query executed successfully",
            sql: query,
            timestamp: new Date().toISOString()
          }
        ]
      });
    } catch (error) {
      console.error('Error executing query:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const renderTabContent = () => {
    switch (activeTab) {
      case 'sql':
        return (
          <div className="space-y-4">
            <div className="bg-gray-900 rounded-lg p-4">
              <textarea
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                placeholder="Enter your SQL query here..."
                className="w-full h-32 bg-transparent text-gray-100 placeholder-gray-400 resize-none focus:outline-none"
              />
            </div>
            <button
              onClick={handleExecute}
              disabled={isLoading || !query.trim()}
              className={cn(
                "px-6 py-2 rounded-lg font-medium transition-colors",
                isLoading || !query.trim()
                  ? "bg-gray-600 text-gray-400 cursor-not-allowed"
                  : "bg-blue-600 hover:bg-blue-700 text-white"
              )}
            >
              {isLoading ? (
                <div className="flex items-center space-x-2">
                  <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
                  <span>Executing...</span>
                </div>
              ) : (
                <div className="flex items-center space-x-2">
                  <Play className="w-4 h-4" />
                  <span>Execute Query</span>
                </div>
              )}
            </button>
          </div>
        );
      
      case 'vector':
        return (
          <div className="space-y-4">
            <div className="bg-gray-900 rounded-lg p-4">
              <input
                type="text"
                placeholder="Enter search query..."
                className="w-full bg-transparent text-gray-100 placeholder-gray-400 focus:outline-none"
              />
            </div>
            <div className="flex space-x-2">
              <input
                type="number"
                placeholder="Limit"
                defaultValue={10}
                className="px-3 py-2 bg-gray-800 rounded-lg text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
              <button className="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors">
                Search
              </button>
            </div>
          </div>
        );
      
      case 'streaming':
        return (
          <div className="space-y-4">
            <div className="bg-gray-900 rounded-lg p-4">
              <input
                type="text"
                placeholder="Enter topic name..."
                className="w-full bg-transparent text-gray-100 placeholder-gray-400 focus:outline-none"
              />
            </div>
            <button className="px-6 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg font-medium transition-colors">
              Subscribe to Stream
            </button>
          </div>
        );
      
      case 'ai':
        return (
          <div className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="bg-gray-900 rounded-lg p-4">
                <h3 className="text-lg font-medium text-gray-100 mb-2">Embedding Model</h3>
                <p className="text-gray-400 text-sm">text-embedding-ada-002</p>
                <div className="mt-2 text-green-400 text-sm">✓ Active</div>
              </div>
              <div className="bg-gray-900 rounded-lg p-4">
                <h3 className="text-lg font-medium text-gray-100 mb-2">Text Generation</h3>
                <p className="text-gray-400 text-sm">llama2</p>
                <div className="mt-2 text-green-400 text-sm">✓ Active</div>
              </div>
            </div>
          </div>
        );
      
      case 'analytics':
        return (
          <div className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="bg-gray-900 rounded-lg p-4 text-center">
                <div className="text-2xl font-bold text-blue-400">3</div>
                <div className="text-gray-400 text-sm">Total Tables</div>
              </div>
              <div className="bg-gray-900 rounded-lg p-4 text-center">
                <div className="text-2xl font-bold text-green-400">5,000</div>
                <div className="text-gray-400 text-sm">Total Rows</div>
              </div>
              <div className="bg-gray-900 rounded-lg p-4 text-center">
                <div className="text-2xl font-bold text-purple-400">5.1 MB</div>
                <div className="text-gray-400 text-sm">Total Size</div>
              </div>
            </div>
          </div>
        );
      
      default:
        return null;
    }
  };

  return (
    <div className="min-h-screen bg-gray-950 text-gray-100">
      {/* Header */}
      <header className="border-b border-gray-800 bg-gray-900/50 backdrop-blur-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <div className="flex items-center space-x-3">
              <div className="w-8 h-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center">
                <Database className="w-5 h-5 text-white" />
              </div>
              <h1 className="text-xl font-bold bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                VectraEdge
              </h1>
            </div>
            
            <div className="flex items-center space-x-4">
              <button className="p-2 text-gray-400 hover:text-gray-100 transition-colors">
                <Settings className="w-5 h-5" />
              </button>
              <button className="p-2 text-gray-400 hover:text-gray-100 transition-colors">
                <HelpCircle className="w-5 h-5" />
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-4 gap-8">
          {/* Sidebar */}
          <div className="lg:col-span-1">
            <div className="bg-gray-900/50 rounded-lg p-4 border border-gray-800">
              <h2 className="text-lg font-semibold text-gray-100 mb-4">Tools</h2>
              <nav className="space-y-2">
                {tabs.map((tab) => {
                  const Icon = tab.icon;
                  return (
                    <button
                      key={tab.id}
                      onClick={() => setActiveTab(tab.id)}
                      className={cn(
                        "w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-left transition-colors",
                        activeTab === tab.id
                          ? "bg-blue-600 text-white"
                          : "text-gray-400 hover:text-gray-100 hover:bg-gray-800"
                      )}
                    >
                      <Icon className="w-5 h-5" />
                      <span>{tab.label}</span>
                    </button>
                  );
                })}
              </nav>
            </div>
          </div>

          {/* Main Content Area */}
          <div className="lg:col-span-3">
            <div className="bg-gray-900/50 rounded-lg p-6 border border-gray-800">
              <div className="flex items-center justify-between mb-6">
                <h2 className="text-2xl font-bold text-gray-100">
                  {tabs.find(t => t.id === activeTab)?.label}
                </h2>
                <div className="flex items-center space-x-2 text-sm text-gray-400">
                  <Terminal className="w-4 h-4" />
                  <span>Ready</span>
                </div>
              </div>

              {renderTabContent()}

              {/* Results */}
              {results && (
                <div className="mt-8">
                  <h3 className="text-lg font-medium text-gray-100 mb-4">Results</h3>
                  <div className="bg-gray-900 rounded-lg p-4 border border-gray-800">
                    <pre className="text-sm text-gray-300 overflow-x-auto">
                      {JSON.stringify(results, null, 2)}
                    </pre>
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
