import test from 'ava'
import { Algo } from '../index'
import { Matrix } from '../index'


test('Algo function from native code', (t) => {
  const algo1 = new Algo('sip');
  t.is(algo1.hash('hello world'), '8170069951894177743')


})

test('Matrix function from native code', (t) => {
  const m1 = new Matrix([[1, 2], [3, 4]]);
  const m2 = new Matrix([[5, 6], [7, 8]]);
  const m3 = m1.mul(m2);
  t.is(m3.display(), '{19 22, 43 50}')
})
