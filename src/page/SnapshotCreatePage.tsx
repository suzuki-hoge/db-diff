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

  const dump: (snapshotName: string, dumpConfigs: DumpConfig[]) => void = (snapshotName: string, dumpConfigs: DumpConfig[]) => {
    invoke('dump_snapshot_command', { snapshotName, dumpConfigJsons: dumpConfigs })
      .then(() => {
        navigate('/snapshot-summary/list')
      })
      .catch((e: string) => {
        toast.dismiss()
        navigate('/error', { state: { message: e } })
      })

    const toastId = toast.loading('保存中...')
    setTimeout(() => {
      watch(toastId, 0)
    }, 500)
  }

  return dumpConfigs.length !== 0 ? <SnapshotCreate dumpConfigs={dumpConfigs} dump={dump} /> : <></>
}

function watch(toastId: string, last: number): void {
  invoke<{ all: number; lines: string[] }>('get_snapshot_processing_status')
    .then((data) => {
      data.lines.slice(last).forEach((line) => {
        toast.success(line, { id: toastId })
      })
      if (data.all !== data.lines.length) {
        setTimeout(() => {
          watch(toastId, data.lines.length)
        }, 10)
      } else {
        toast.success('保存しました', { id: toastId })
      }
    })
    .catch(() => {
      toast.dismiss()
    })
}
