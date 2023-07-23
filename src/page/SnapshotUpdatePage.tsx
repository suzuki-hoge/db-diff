import { type FC } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { type SnapshotSummary } from '../types'
import { useLocation, useNavigate } from 'react-router-dom'
import { SnapshotUpdate } from '../components/templates/snapshot-update/SnapshotUpdate'
import { toast } from 'react-hot-toast'

export const SnapshotUpdatePage: FC = () => {
  const location = useLocation()
  const navigate = useNavigate()

  const snapshotSummary = location.state as SnapshotSummary

  const update: (snapshotSummary: SnapshotSummary) => void = (snapshotSummary) => {
    invoke('update_snapshot_summary_command', { snapshotSummaryJson: snapshotSummary })
      .then(() => {
        toast.success('保存しました')
        navigate('/snapshot-summary/list')
      })
      .catch((e: string) => {
        navigate('/error', { state: { message: e } })
      })
  }

  return snapshotSummary !== undefined ? <SnapshotUpdate snapshotSummary={snapshotSummary} update={update} /> : <></>
}
