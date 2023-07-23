import { type FC, useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { type SnapshotSummary } from '../types'
import { useLocation, useNavigate } from 'react-router-dom'
import { SnapshotList } from '../components/templates/snapshot-list/SnapshotList'
import { toast } from 'react-hot-toast'

export const SnapshotListPage: FC = () => {
  const [snapshotSummaries, setSnapshotSummaries] = useState<SnapshotSummary[]>([])

  const navigate = useNavigate()
  const location = useLocation()

  useEffect(() => {
    invoke<SnapshotSummary[]>('all_snapshot_summaries_command')
      .then((data) => {
        setSnapshotSummaries(data)
      })
      .catch((e: string) => {
        navigate('/error', { state: { message: e } })
      })
  }, [location])

  const remove: (snapshotId: string) => void = (snapshotId) => {
    invoke('delete_snapshot_summary_command', { snapshotId })
      .then(() => {
        toast.success('削除しました')
        navigate('/snapshot-summary/list')
      })
      .catch((e: string) => {
        navigate('/error', { state: { message: e } })
      })
  }

  return <SnapshotList snapshotSummaries={snapshotSummaries} remove={remove} />
}
