import http from 'k6/http'
import { check } from 'k6'

export function makeHttpRequest() {
  const res = http.get(__ENV.SERVER_ENDPOINT || 'http://localhost:3002/')

  if (res.status !== 200) {
    console.error(`‼️ Failed to run HTTP request: ${res.status}`)
  }

  check(res, {
    'response code was 200': (r) => r.status == 200,
    "response body is 'Hello World'": (r) => r.body === 'Hello World',
  })
}
