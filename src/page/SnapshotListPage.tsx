import { type FC, useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { type SnapshotSummary } from '../types'
import { useLocation, useNavigate } from 'react-router-dom'
import { SnapshotList } from '../components/templates/snapshot-list/SnapshotList'

export const SnapshotListPage: FC = () => {
  const [snapshotSummaries, setSnapshotSummaries] = useState<SnapshotSummary[]>([])

  const navigate = useNavigate()
  const location = useLocation()

  useEffect(() => {
    invoke<SnapshotSummary[]>('all_snapshot_summaries_command')
      .then((data) => {
        setSnapshotSummaries(data)
      })
      .catch(console.log)
  }, [location])

  const remove: (snapshotId: string) => void = (snapshotId) => {
    invoke('delete_snapshot_summary_command', { snapshotId })
      .then(() => {
        navigate('/snapshot-summary/list')
      })
      .catch(console.log)
  }

  return <SnapshotList snapshotSummaries={snapshotSummaries} remove={remove} />
}
