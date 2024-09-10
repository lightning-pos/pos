'use client';

import React, { useState, useEffect } from 'react';
import { powerSyncDb } from '@/components/providers/system_provider';

const Console = () => {
  const [query, setQuery] = useState('');
  const [result, setResult] = useState<any[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [tables, setTables] = useState<string[]>([]);
  const [selectedTable, setSelectedTable] = useState<string | null>(null);
  const [page, setPage] = useState(1);
  const [pageSize, setPageSize] = useState(10);
  const [totalCount, setTotalCount] = useState(0);
  const [activeTab, setActiveTab] = useState<'console' | 'tables'>('console');

  useEffect(() => {
    fetchTables();
  }, []);

  const fetchTables = async () => {
    try {
      const tablesResult = await powerSyncDb.execute("SELECT name FROM sqlite_master WHERE type='table' AND name LIKE 'ps_data__%'");
      if (tablesResult.rows) {
        let tableNames: string[] = [];
        if (Array.isArray(tablesResult.rows)) {
          tableNames = tablesResult.rows.map((row: any) => row.name);
        } else if (typeof tablesResult.rows.item === 'function') {
          for (let i = 0; i < tablesResult.rows.length; i++) {
            tableNames.push(tablesResult.rows.item(i).name);
          }
        }
        // Process table names to remove "ps_data__" prefix
        const processedTableNames = tableNames.map(name =>
          name.replace('ps_data__', '')
        );
        setTables(processedTableNames);
      }
    } catch (err) {
      console.error('Error fetching tables:', err);
    }
  };

  const executeQuery = async (customQuery?: string) => {
    try {
      setError(null);
      const queryToExecute = customQuery || query;
      const queryResult = await powerSyncDb.execute(queryToExecute);
      let rows: any[] = [];

      if (queryResult.rows) {
        if (Array.isArray(queryResult.rows)) {
          rows = queryResult.rows;
        } else if (typeof queryResult.rows === 'object' && 'length' in queryResult.rows && typeof queryResult.rows.item === 'function') {
          rows = Array.from({ length: queryResult.rows.length }, (_, i) => queryResult.rows!.item(i));
        }
      }

      setResult(rows);
      if (!customQuery) {
        // Only reset these for manual queries, not for table data fetches
        setSelectedTable(null);
        setTotalCount(rows.length);
        setPage(1);
      }
    } catch (err) {
      console.error('Query execution error:', err);
      setError(err instanceof Error ? err.message : String(err));
      setResult(null);
    }
  };

  const handleTableClick = async (table: string) => {
    const fullTableName = `ps_data__${table}`;
    setSelectedTable(table);
    const countQuery = `SELECT COUNT(*) as count FROM ${fullTableName}`;
    const countResult = await powerSyncDb.execute(countQuery);
    if (countResult.rows && countResult.rows.length > 0) {
      const firstRow = Array.isArray(countResult.rows)
        ? countResult.rows[0]
        : countResult.rows.item(0);
      setTotalCount(firstRow.count);
    }
    setPage(1);
    fetchTableData(table, 1);
  };

  const fetchTableData = async (table: string, pageNum: number) => {
    const offset = (pageNum - 1) * pageSize;
    executeQuery(`SELECT * FROM ${table} LIMIT ${pageSize} OFFSET ${offset}`);
  };

  const handlePageChange = (newPage: number) => {
    setPage(newPage);
    if (selectedTable) {
      fetchTableData(selectedTable, newPage);
    }
  };

  return (
    <div className="grid grid-cols-1 md:grid-cols-4 lg:grid-cols-5 h-screen bg-gray-900 text-white">
      <aside className="md:col-span-1 bg-gray-800 p-4 overflow-y-auto flex flex-col">
        <div className="flex mb-4">
          <button
            onClick={() => setActiveTab('console')}
            className={`flex-1 py-2 ${activeTab === 'console' ? 'bg-gray-700' : 'bg-gray-800'} text-white rounded-l focus:outline-none`}
          >
            Console
          </button>
          <button
            onClick={() => setActiveTab('tables')}
            className={`flex-1 py-2 ${activeTab === 'tables' ? 'bg-gray-700' : 'bg-gray-800'} text-white rounded-r focus:outline-none`}
          >
            Tables
          </button>
        </div>
        {activeTab === 'console' && (
          <div className="flex-grow flex flex-col">
            <textarea
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              className="w-full h-48 p-2 mb-4 bg-gray-700 text-white border border-gray-600 rounded"
              placeholder="Enter SQL query"
            />
            <button
              onClick={() => executeQuery()}
              className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 mb-2"
            >
              Run Query
            </button>
            <button
              onClick={() => setQuery('')}
              className="px-4 py-2 bg-gray-700 text-white rounded hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500"
            >
              Clear
            </button>
          </div>
        )}
        {activeTab === 'tables' && (
          <ul className="flex-grow">
            {tables.map((table) => (
              <li key={table} className="mb-2">
                <button
                  onClick={() => handleTableClick(table)}
                  className="w-full text-left px-2 py-1 rounded hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  {table}
                </button>
              </li>
            ))}
          </ul>
        )}
      </aside>
      <main className="md:col-span-3 lg:col-span-4 flex flex-col p-4 overflow-auto">
        <h1 className="text-2xl font-bold mb-4">Database Explorer</h1>
        {selectedTable && <h2 className="text-xl font-bold mb-2">{selectedTable}</h2>}
        {renderResultTable()}
      </main>
    </div>
  );

  function renderResultTable() {
    if (error) {
      return <div className="text-red-500 mb-4">Error: {error}</div>;
    }

    if (!result || result.length === 0) {
      return <div>No results found.</div>;
    }

    const headers = Object.keys(result[0]);

    return (
      <div>
        <div className="overflow-x-auto">
          <table className="min-w-full bg-gray-800">
            <thead>
              <tr>
                {headers.map((header) => (
                  <th key={header} className="px-4 py-2 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                    {header}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {result.map((row, rowIndex) => (
                <tr key={rowIndex} className={rowIndex % 2 === 0 ? 'bg-gray-800' : 'bg-gray-750'}>
                  {headers.map((header) => (
                    <td key={header} className="px-4 py-2 whitespace-nowrap text-sm">
                      {String(row[header])}
                    </td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        <div className="mt-4 flex flex-col sm:flex-row justify-between items-center">
          <span className="mb-2 sm:mb-0">
            Showing {(page - 1) * pageSize + 1} to {Math.min(page * pageSize, totalCount)} of {totalCount} results
          </span>
          <div>
            <button
              onClick={() => handlePageChange(page - 1)}
              disabled={page === 1}
              className="px-3 py-1 bg-gray-700 text-white rounded hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500 disabled:opacity-50 mr-2"
            >
              Previous
            </button>
            <button
              onClick={() => handlePageChange(page + 1)}
              disabled={page * pageSize >= totalCount}
              className="px-3 py-1 bg-gray-700 text-white rounded hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500 disabled:opacity-50"
            >
              Next
            </button>
          </div>
        </div>
      </div>
    );
  }
};

export default Console;
