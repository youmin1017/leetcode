class Solution {
public:
    vector<vector<int>> floodFill(vector<vector<int>>& image, int sr, int sc, int newColor) { 
        if(image[sr][sc] == newColor) return image;
        helper(image, sr, sc, newColor, image[sr][sc]);
        return image;
    }
    void helper(vector<vector<int>>& image, int i, int j, int newColor, int oldColor) {
        if(i < 0 or j < 0 or i >= image.size() or j >= image[0].size() or image[i][j] != oldColor) return;
        image[i][j] = newColor;
        helper(image, i-1, j, newColor, oldColor);
        helper(image, i, j-1, newColor, oldColor);
        helper(image, i+1, j, newColor, oldColor);
        helper(image, i, j+1, newColor, oldColor);
    }
};
