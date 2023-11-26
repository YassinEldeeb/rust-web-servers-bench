import { makeHttpRequest, handleBenchmarkSummary } from './k6.shared.js'

const startVUs = __ENV.START_VUS ? parseInt(__ENV.START_VUS) : 50
const peakVUs = __ENV.BENCH_VUS ? parseInt(__ENV.BENCH_VUS) : 500
const testDuration = __ENV.BENCH_OVER_TIME || '3m'

export const options = {
  scenarios: {
    ramping_test: {
      executor: 'ramping-vus',
      startVUs: startVUs,
      stages: [
        { duration: '30s', target: peakVUs }, // ramp up to the peak load
        { duration: testDuration, target: peakVUs }, // hold at the peak load for the test duration
        { duration: '30s', target: 0 }, // ramp down to 0 VUs
      ],
    },
  },
}

export default function () {
  makeHttpRequest()
}

export function handleSummary(data) {
  return handleBenchmarkSummary(data, { peakVUs, duration: testDuration })
}
