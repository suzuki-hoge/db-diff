import { type FC, useEffect, useRef, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { type SnapshotDiff } from '../types'
import { useLocation, useNavigate } from 'react-router-dom'
import { DiffViewer } from '../components/templates/diff-viewer/DiffViewer'
import { toast } from 'react-hot-toast'

export const DiffPage: FC = () => {
  const [snapshotDiff, setSnapshotDiff] = useState<SnapshotDiff | undefined>(undefined)

  const navigate = useNavigate()
  const location = useLocation()
  const isFirstMount = useRef(true)

  const { snapshotId1, snapshotId2 } = location.state as { snapshotId1: string; snapshotId2: string }

  useEffect(() => {
    invoke<SnapshotDiff>('find_snapshot_diff_command', { snapshotId1, snapshotId2 })
      .then((data) => {
        setSnapshotDiff(data)
      })
      .catch((e: string) => {
        if (e === 'snapshot diff not created') {
          if (process.env.NODE_ENV === 'development') {
            if (isFirstMount.current) {
              isFirstMount.current = false
              return
            }
          }

          toast.promise(
            invoke<SnapshotDiff>('create_snapshot_diff_command', { snapshotId1, snapshotId2 }),
            { loading: '差分を計算中...', success: '差分を保存しました', error: '計算に失敗しました' },
            { style: { minWidth: '200px' } },
          )
            .then((data) => {
              setSnapshotDiff(data)
            })
            .catch((e: string) => {
              navigate('/error', { state: { message: e } })
            })
        } else {
          navigate('/error', { state: { message: e } })
        }
      })
  }, [snapshotId1, snapshotId2])

  return snapshotDiff !== undefined ? <DiffViewer tableDiffs={snapshotDiff.tableDiffs} ignoreTableNames={[]} /> : <></>
}
