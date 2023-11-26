import { makeHttpRequest, handleBenchmarkSummary } from './k6.shared.js'

const vus = __ENV.BENCH_VUS ? parseInt(__ENV.BENCH_VUS) : 200
const testDuration = __ENV.BENCH_OVER_TIME || '3m'

export const options = {
  stages: [
    { duration: '30s', target: vus }, // ramp-up to the target VU count
    { duration: testDuration, target: vus }, // stay at target VU count for the test duration
  ],
}

export default function () {
  makeHttpRequest()
}

export function handleSummary(data) {
  return handleBenchmarkSummary(data, { vus, duration: testDuration })
}
