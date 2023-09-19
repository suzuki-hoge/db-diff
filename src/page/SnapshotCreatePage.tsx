import { type FC, useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { useNavigate } from 'react-router-dom'
import { SnapshotCreate } from '../components/templates/snapshot-create/SnapshotCreate'
import { toast } from 'react-hot-toast'
import { type DumpConfig } from '../types'

export const SnapshotCreatePage: FC = () => {
  const [dumpConfigs, setDumpConfigs] = useState<DumpConfig[]>([])

  const navigate = useNavigate()

  useEffect(() => {
    invoke<DumpConfig[]>('find_recent_dump_configs_command')
      .then((data) => {
        setDumpConfigs(data)
      })
      .catch((e: string) => {
        navigate('/error', { state: { message: e } })
      })
  }, [location])

  const dump: (snapshotId: string, snapshotName: string, dumpConfigs: DumpConfig[]) => void = (
    snapshotId: string,
    snapshotName: string,
    dumpConfigs: DumpConfig[]
  ) => {
    invoke('dump_snapshot_command', { snapshotId, snapshotName, dumpConfigJsons: dumpConfigs })
      .then(() => {
        navigate('/snapshot-summary/list')
      })
      .catch((e: string) => {
        toast.dismiss()
        navigate('/error', { state: { message: e } })
      })

    watch(snapshotId, -1)
  }

  return dumpConfigs.length !== 0 ? <SnapshotCreate dumpConfigs={dumpConfigs} dump={dump} /> : <></>
}

function watch(snapshotId: string, lastPercent: number): void {
  invoke<{ percent: number; done: number; total: number; status: string }>('get_snapshot_result_command', { snapshotId })
    .then((data) => {
      if (lastPercent !== data.percent) {
        toast.success(`${data.percent}% done.`)
      }
      if (data.status === 'queued' || data.status === 'processing') {
        setTimeout(() => {
          watch(snapshotId, data.percent)
        }, 300)
      }
    })
    .catch(() => {
      toast.dismiss()
    })
}
