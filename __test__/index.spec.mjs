import test from 'ava'
import { resolve } from '../index.js'

test('resolve package info', (t) => {
  const packageInfo = resolve('lodash')
  t.not(packageInfo, null)
  if (packageInfo) {
    t.is(packageInfo.name, 'lodash')
    t.is(typeof packageInfo.version, 'string')
    t.is(typeof packageInfo.description, 'string')
    t.true(Array.isArray(packageInfo.keywords))
    t.is(typeof packageInfo.path, 'string')
    t.is(typeof packageInfo.homepage, 'string')
    t.is(typeof packageInfo.license, 'string')
    t.is(typeof packageInfo.author, 'string')
  }
})