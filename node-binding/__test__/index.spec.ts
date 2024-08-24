import test from 'ava'
import { Algo } from '../index'

test('sync function from native code', (t) => {
  const algo1 = new Algo('sip');
  t.is(algo1.hash('hello world'), '8170069951894177743')
})
