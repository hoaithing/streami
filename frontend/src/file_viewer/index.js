import React, { useState } from 'react';

const FileContentViewer = () => {
    const [fileName, setFileName] = useState('');
    const [line, setLine] = useState('');
    const [numLines, setNumLines] = useState(100);
    const [searchTerm, setSearchTerm] = useState('');
    const [fileContent, setFileContent] = useState([]);
    const [fileInfo, setFileInfo] = useState(null);
    const [searchResults, setSearchResults] = useState(null);
    const [error, setError] = useState(null);

    const fetchFileContent = async (e) => {
        e.preventDefault();

        try {
            // Reset previous states
            setError(null);
            setFileContent([]);
            setFileInfo(null);
            setSearchResults(null);

            // Construct query parameters
            const params = new URLSearchParams({
                file_name: fileName,
                num_lines: numLines.toString()
            });

            // Add line parameter if it exists
            if (line) {
                params.append('line', line);
            }

            // Add search term if it exists
            if (searchTerm) {
                params.append('search', searchTerm);
            }

            // Fetch file content
            const response = await fetch(`http://localhost:8080/api/file-content?${params.toString()}`, {
                method: 'GET',
                headers: {
                    'Accept': 'application/json'
                }
            });

            if (!response.ok) {
                // Handle non-200 responses
                const errorText = await response.text();
                throw new Error(errorText || 'Failed to fetch file content');
            }

            const data = await response.json();

            setFileContent(data.content);
            setFileInfo({
                totalLines: data.total_lines,
                startLine: data.start_line,
                endLine: data.end_line
            });

            // Set search results if available
            if (data.search_results) {
                setSearchResults(data.search_results);
            }
        } catch (err) {
            setError(err.message || 'An error occurred');
        }
    };

    return (
        <div className="container mx-auto p-6 max-w-4xl">
            <h1 className="text-2xl font-bold mb-4">File Content Viewer</h1>

            <form onSubmit={fetchFileContent} className="mb-4 space-y-2">
                <div className="flex space-x-4">
                    <input
                        type="text"
                        placeholder="File Name"
                        value={fileName}
                        onChange={(e) => setFileName(e.target.value)}
                        className="flex-grow p-2 border rounded"
                        required
                    />

                    <input
                        type="number"
                        placeholder="Start Line (optional)"
                        value={line}
                        onChange={(e) => setLine(e.target.value)}
                        className="w-32 p-2 border rounded"
                        min="1"
                    />

                    <input
                        type="number"
                        placeholder="Lines to Show"
                        value={numLines}
                        onChange={(e) => setNumLines(e.target.value)}
                        className="w-32 p-2 border rounded"
                        min="1"
                    />
                </div>

                <div className="flex space-x-4">
                    <input
                        type="text"
                        placeholder="Search in file"
                        value={searchTerm}
                        onChange={(e) => setSearchTerm(e.target.value)}
                        className="flex-grow p-2 border rounded"
                    />

                    <button
                        type="submit"
                        className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
                    >
                        View File
                    </button>
                </div>
            </form>

            {error && (
                <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
                    {error}
                </div>
            )}

            {fileInfo && (
                <div className="mb-4 text-gray-600">
                    Total Lines: {fileInfo.totalLines} |
                    Showing Lines: {fileInfo.startLine} - {fileInfo.endLine}
                </div>
            )}

            {searchResults && (
                <div className="mb-4">
                    <h2 className="text-lg font-semibold">Search Results</h2>
                    <ul className="bg-gray-50 p-2 rounded">
                        {searchResults.map(([lineNumber, line], index) => (
                            <li
                                key={index}
                                className="py-1 border-b last:border-b-0 hover:bg-gray-100 cursor-pointer"
                            >
                                <span className="font-mono text-gray-600 mr-2">Line {lineNumber}:</span>
                                {line}
                            </li>
                        ))}
                    </ul>
                </div>
            )}

            {fileContent.length > 0 && (
                <pre className="bg-gray-100 p-4 rounded overflow-x-auto">
          {fileContent.map((line, index) => (
              <code key={index} className="block">
                  {line}
              </code>
          ))}
        </pre>
            )}
        </div>
    );
};

export default FileContentViewer;