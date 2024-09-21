'use client'
import { pgliteDb } from '@/components/providers/system_provider'
import { Repl } from '@electric-sql/pglite-repl'

const Pglite = () => {
  return (
    <Repl pg={pgliteDb} />
  )
}

export default Pglite
