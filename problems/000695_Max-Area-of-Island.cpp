class Solution {
public:
  int maxAreaOfIsland(vector<vector<int>> &grid) {
    int maxSize = 0, m = grid.size(), n = grid[0].size();
    for (int i = 0; i < m; ++i) {
      for (int j = 0; j < n; ++j) {
        if (grid[i][j] == 1) {
          int size = 0; // The Size that Pass By Reference
          maxSize = std::max(maxSize, helper(grid, i, j, size));
        }
      }
    }
    return maxSize;
  }
  /**
   * @return size of land
   * Using DFS to throughout all lands
   */
  int helper(vector<vector<int>> &grid, int i, int j, int &currentSize) {
    if (i < 0 or i >= grid.size() or j < 0 or j >= grid[0].size())
      return 0;
    if (grid[i][j] == 1) {
      grid[i][j] = 0;
      currentSize++;
      helper(grid, i + 1, j, currentSize);
      helper(grid, i, j + 1, currentSize);
      helper(grid, i - 1, j, currentSize);
      helper(grid, i, j - 1, currentSize);
    }
    return currentSize;
  }
};
