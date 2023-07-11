import { type FC, useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { type SnapshotDiff } from '../types'
import { useLocation } from 'react-router-dom'
import { DiffViewer } from '../components/templates/diff-viewer/DiffViewer'

export const DiffPage: FC = () => {
  const [snapshotDiff, setSnapshotDiff] = useState<SnapshotDiff | undefined>(undefined)

  const location = useLocation()

  const { snapshotId1, snapshotId2 } = location.state as { snapshotId1: string; snapshotId2: string }

  useEffect(() => {
    invoke<SnapshotDiff>('find_snapshot_diff_command', { snapshotId1, snapshotId2 })
      .then((data) => {
        setSnapshotDiff(data)
      })
      .catch(console.log)
  }, [snapshotId1, snapshotId2])

  return snapshotDiff !== undefined ? <DiffViewer tableDiffs={snapshotDiff.tableDiffs} ignoreTableNames={[]} /> : <></>
}
