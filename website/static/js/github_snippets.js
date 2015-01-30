/*
 * Script that retrieves a random snippet of code from GitHub using their API.
 */
(function() {
    'use strict';
    var GITHUB_USERNAME = 'euclio';

    /*
     * Returns a random element from the given array. Optionally, you may
     * provide a maximum index which should be returned from this function.
     * Otherwise, the entire array is used.
     */
    function randomElement(array) {
        return array[Math.floor(Math.random() * array.length)];
    }

    /*
     * Writes lines of code into the page.
     */
    function appendLines(lines) {
        document.querySelector('#code code').textContent = lines.join('\n');
    }

    /*
     * Sends an API request to GitHub.
     */
    function sendApiRequest(endpoint, callback) {
        var API_SERVER= 'https://api.github.com';
        var request = new XMLHttpRequest();
        request.open('GET', API_SERVER + endpoint);
        request.onreadystatechange = function() {
            if (request.readyState === 4) {
                if (request.status === 200) {
                    callback(null, JSON.parse(request.responseText));
                } else {
                    var err = new Error(
                            "The server responded with a non-200 error code.");
                    err.request = request;
                    callback(err);
                }
            }
        };
        request.send();
        return request;
    }

    // Github requires that we have a search term when searching, so we will
    // just provide common programming keywords and select randomly from them.
    var KEYWORDS = [
        'char',
        'else',
        'for',
        'if',
        'int',
        'main',
        'print',
        'val',
        'var',
    ];

    async.waterfall([
        // First, we do a code search on the username to retrieve a list of
        // search results.
        // TODO: This endpoint has an extremely low rate limit, so we should
        // probably cache the results to once per hour.
        function searchCode(callback) {
            var keyword = randomElement(KEYWORDS);
            var endpoint =
                '/search/code?q=user:' + GITHUB_USERNAME + '+' + keyword;
            sendApiRequest(endpoint, function(err, response) {
                callback(err, response);
            });
        },

        function retrieveFileData(response, callback) {
            var files = response.items;
            var file = randomElement(files);
            var endpoint =
                    '/repos/' + GITHUB_USERNAME + '/' + file.repository.name +
                    '/contents/' + file.path;
            sendApiRequest(endpoint, function(err, response) {
                callback(err, response);
            });
        },

        function getRandomLines(response, callback) {
            var fileData = response.content;
            var fileContent = atob(fileData);
            var lines = fileContent.split('\n');

            // Ideally, we will get the number of lines we want from this file.
            // However, if the file is smaller than what we request lines, then
            // we will just use all of the lines.
            var LINES_TO_GET = 10;
            if (lines.length <= LINES_TO_GET) {
                return callback(null, lines);
            }

            var firstIndex = Math.floor(
                    Math.random() * (lines.length - LINES_TO_GET));
            callback(null, lines.slice(firstIndex, firstIndex + LINES_TO_GET));
        }
    ], function(err, lines) {
        if (!err) {
            if (document.readyState == 'loading') {
                document.addEventListener('DOMContentLoaded', function(e) {
                    appendLines(lines);
                });
            } else {
                appendLines(lines);
            }
        } else {
            if (err.request.status === 403) {
                appendLines([
                        "You've hit the rate limit for the GitHub API.",
                        "Please try again later."
                ]);
            }
        }
    });
})();
