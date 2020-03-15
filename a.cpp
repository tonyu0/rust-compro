#include <iostream>
#include <vector>
#include <queue>
#include <limits>
using namespace std;
constexpr long long INF = 1LL << 60;

template <typename T>
class MinCostFlow
{
public:
  MinCostFlow(int n) : n(n), capacity(n, vector<T>(n)), cost(n, vector<T>(n)), edges(n), prev(n) {}

  void add_edge(int src, int dst, T cap, T cos)
  {
    capacity[src][dst] = cap;
    capacity[dst][src] = 0;
    cost[src][dst] = cos;
    cost[dst][src] = -cos;
    edges[src].push_back(dst);
    edges[dst].push_back(src);
  }

  T min_cost_flow(int s, int t, T f)
  {
    T res = 0;
    h.assign(n, 0);
    while (f > 0)
    {
      // 残余グラフ上で辿り着けなかったら無理
      if (!dijkstra(s, t))
        return -1;
      // ポテンシャルを計算する。
      for (int i = 0; i < n; ++i)
        h[i] += min_cost[i];

      T d = f;
      for (int v = t; v != s; v = prev[v])
        d = min(d, capacity[prev[v]][v]);
      // 今のステップではd流せるので、流す。
      f -= d;
      res += d * h[t];
      // 残余グラフを編集
      for (int v = t; v != s; v = prev[v])
      {
        capacity[prev[v]][v] -= d;
        capacity[v][prev[v]] += d;
      }
    }
    return res;
  }

private:
  int n;
  T inf = numeric_limits<T>::max() / 3;
  vector<vector<T>> capacity, cost;
  vector<vector<int>> edges;
  vector<T> min_cost, h;
  vector<int> prev;

  bool dijkstra(int s, int t)
  {
    min_cost.assign(n, inf);
    min_cost[s] = 0;
    priority_queue<pair<T, int>, vector<pair<T, int>>, greater<pair<T, int>>> pq;
    pq.emplace(0, s);
    while (!pq.empty())
    {
      int c = pq.top().first;
      int v = pq.top().second;
      pq.pop();
      if (min_cost[v] < c)
        continue;
      for (int nv : edges[v])
      {
        if (capacity[v][nv] > 0 && min_cost[nv] > min_cost[v] + cost[v][nv] + h[v] - h[nv])
        {
          min_cost[nv] = min_cost[v] + cost[v][nv] + h[v] - h[nv];
          prev[nv] = v;
          pq.emplace(min_cost[nv], nv);
        }
      }
    }
    return min_cost[t] != inf;
  }
};
int main()
{
  long long N, A[100010], M, B[100010], Q, T, C, D;
  cin >> N;
  for (int i = 0; i < N; ++i)
    cin >> A[i];
  cin >> M;
  for (int i = 0; i < M; ++i)
    cin >> B[i];
  MinCostFlow<long long> flow(N + M + 2);
  int s = N + M, t = N + M + 1;
  // B -> A
  for (int i = 0; i < M; ++i)
  {
    for (int j = 0; j < N; ++j)
      flow.add_edge(i, M + j, INF, abs(A[j] - B[i]));
  }
  for (int i = 0; i < M - 1; ++i)
  {
    flow.add_edge(M + i, M + i + 1, INF, abs(A[i] - A[i + 1]));
    flow.add_edge(M + i + 1, M + i, INF, abs(A[i] - A[i + 1]));
  }

  for (int i = 0; i < M; ++i)
    flow.add_edge(s, i, INF, 0);
  for (int i = 0; i < N; ++i)
    flow.add_edge(M + i, t, 1, 0);
  cout << flow.min_cost_flow(s, t, N) << '\n';

  cin >> Q;
  for (int i = 0; i < Q; ++i)
  {
    cin >> T >> C >> D;
    cout << 0 << '\n';
  }
  return 0;
}