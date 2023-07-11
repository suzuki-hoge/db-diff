import { type FC } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { useNavigate } from 'react-router-dom'
import { SnapshotCreate } from '../components/templates/snapshot-create/SnapshotCreate'

export const SnapshotCreatePage: FC = () => {
  const navigate = useNavigate()

  const dump: (snapshotName: string) => void = (snapshotName) => {
    invoke('dump_snapshot_command', { snapshotName })
      .then(() => {
        navigate('/snapshot-summary/list')
      })
      .catch(console.log)
  }

  return <SnapshotCreate dump={dump} />
}
