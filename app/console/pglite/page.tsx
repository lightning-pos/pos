'use client'
import { pgliteWorker } from '@/components/providers/system_provider'
import { Repl } from '@electric-sql/pglite-repl'

const Pglite = () => {
  return (
    <Repl pg={pgliteWorker} />
  )
}

export default Pglite
