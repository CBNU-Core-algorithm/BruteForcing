#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main() {
    int n, m;
    cin >> n >> m;
    vector<string> board(n);
    for (int i = 0; i < n; i++) {
        cin >> board[i];
    }

    int min = 64;
    for (int i = 0; i < n - 7; i++) {
        for (int j = 0; j < m - 7; j++) {
            int count = 0;
            for (int k = i; k < i + 8; k++) {
                for (int l = j; l < j + 8; l++) {
                    if ((k + l) % 2 == 0) {
                        if (board[k][l] == 'B') {
                            count++;
                        }
                    }
                    else {
                        if (board[k][l] == 'W') {
                            count++;
                        }
                    }
                }
            }
            if (count > 32) {
                count = 64 - count;
            }
            if (count < min) {
                min = count;
            }
        }
    }
    cout << min;
}