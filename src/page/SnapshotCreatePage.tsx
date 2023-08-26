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
    toast
      .promise(
        invoke('dump_snapshot_command', { snapshotName, dumpConfigJsons: dumpConfigs }),
        { loading: 'スナップショットを作成中...', success: '保存しました', error: 'エラーが発生しました' },
        { style: { minWidth: '200px' } }
      )
      .then(() => {
        navigate('/snapshot-summary/list')
      })
      .catch((e: string) => {
        navigate('/error', { state: { message: e } })
      })
  }

  return dumpConfigs.length !== 0 ? <SnapshotCreate dumpConfigs={dumpConfigs} dump={dump} /> : <></>
}
