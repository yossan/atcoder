#include <iostream>
#include <queue>
using namespace std;

int main() {
	int n, m;
	cin >> n >> m;
	vector<vector<int>> graph(n, vector<int>());
	for (int i = 0; i < m; i++) {
		int u, v;
		cin >> u >> v;
		u--; v--;
		graph[u].push_back(v);
		graph[v].push_back(u);
	}
	vector<bool> used(n);
	int s = 0;
	for (int i = 0; i < n; i++) {
		if (!used[i]) {
			s++;
			printf("i = %d, s = %d\n", i, s);
			queue<int> que;
			que.push(i);
			while (!que.empty()) {
				int q = que.front(); que.pop();
				for (int v : graph[q]) {
					printf("pos = %d ", v);
					if (!used[v]) {
						used[v] = true;
						que.push(v);
					}
				}
			}
			printf("\n");
		}
	}
	cout << "m =" << m << '\n';
	cout << "n =" << n << '\n';
	cout << "s =" << s << '\n';
	cout << m - n + s << '\n';
}

