const lines = `4 10
2 7
7 1
2 9
4 7`.split("\n");

const [N, x] = lines.shift().split(" ").map(Number);
let w = [];
let r = [];
lines.forEach((l) => {
  const num = l.split(" ").map(Number);
  w.push(num[0]);
  r.push(num[1]);
});
let dp = new Array(N + 1).fill(new Array(x + 1).fill(0));
dp[0][0] = 0;
for (let i = 1; i <= N; i++) {
  for (let j = 0; j < x; j++) {
    if (j < w[i]) {
      dp[i][j] = dp[i - 1][j];
    } else {
      dp[i][j] = Math.max(dp[i - 1][j] || 0, dp[i - 1][j - w[i]] + r[i] || 0);
    }
  }
}
let answer = 0;
for (let i = 0; i < x; i++) {
  answer = Math.max(answer, dp[N][i]);
}
console.log(answer);
