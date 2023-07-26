import { type FC } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { useNavigate } from 'react-router-dom'
import { SnapshotCreate } from '../components/templates/snapshot-create/SnapshotCreate'
import { toast } from 'react-hot-toast'

export const SnapshotCreatePage: FC = () => {
  const navigate = useNavigate()

  const dump: (snapshotName: string) => void = (snapshotName) => {
    toast
      .promise(
        invoke('dump_snapshot_command', { snapshotName }),
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

  return <SnapshotCreate dump={dump} />
}
