import { type FC, useState } from 'react'
import styles from './SnapshotList.module.scss'
import { TextCard } from '../../molecules/text-card/TextCard'
import { type SnapshotSummary } from '../../../types'
import { Header } from '../../molecules/header/Header'
import { IconBack } from '../../atoms/icon-back/IconBack'
import { IconPlus } from '../../atoms/icon-plus/IconPlus'
import { IconGear } from '../../atoms/icon-gear/IconGear'
import { IconEdit } from '../../atoms/icon-edit/IconEdit'
import { IconDelete } from '../../atoms/icon-delete/IconDelete'
import { useNavigate } from 'react-router-dom'

interface Props {
  snapshotSummaries: SnapshotSummary[]
  remove: (id: string) => void
}

export const SnapshotList: FC<Props> = (props) => {
  const [isSetting, setIsSetting] = useState(false)
  const [selectedSnapshotSummary, setSelectedSnapshotSummary] = useState<SnapshotSummary | null>(null)

  const navigate = useNavigate()

  return (
    <div className={styles.template}>
      <Header
        globals={
          <IconBack
            variant={'large'}
            onClick={() => {
              navigate('/project/list')
            }}
          />
        }
        locals={
          <>
            <IconPlus
              variant={'large'}
              onClick={() => {
                navigate('/snapshot-summary/create')
              }}
            />
            <IconGear
              variant={'large'}
              onClick={() => {
                setIsSetting(!isSetting)
              }}
            />
          </>
        }
      />
      <div className={styles.component}>
        <div className={styles.snapshots}>
          {props.snapshotSummaries.map((snapshotSummary) => (
            <div key={snapshotSummary.snapshotId} className={styles.item}>
              <TextCard
                key={snapshotSummary.snapshotId}
                label={snapshotSummary.snapshotName}
                text={snapshotSummary.createAt}
                selected={selectedSnapshotSummary?.snapshotId === snapshotSummary.snapshotId}
                onClick={() => {
                  if (selectedSnapshotSummary === null) {
                    setSelectedSnapshotSummary(snapshotSummary)
                  } else if (selectedSnapshotSummary.snapshotId === snapshotSummary.snapshotId) {
                    setSelectedSnapshotSummary(null)
                  } else {
                    if (selectedSnapshotSummary.createAt < snapshotSummary.createAt) {
                      const snapshotId1 = selectedSnapshotSummary.snapshotId
                      const snapshotId2 = snapshotSummary.snapshotId
                      navigate('/diff', { state: { snapshotId1, snapshotId2 } })
                    } else {
                      const snapshotId1 = snapshotSummary.snapshotId
                      const snapshotId2 = selectedSnapshotSummary.snapshotId
                      navigate('/diff', { state: { snapshotId1, snapshotId2 } })
                    }
                  }
                }}
              />
              {isSetting && (
                <div className={styles.icons}>
                  <IconEdit
                    variant={'medium'}
                    onClick={() => {
                      navigate('/snapshot-summary/update', { state: snapshotSummary })
                    }}
                  />
                  <IconDelete
                    variant={'medium'}
                    onClick={() => {
                      props.remove(snapshotSummary.snapshotId)
                    }}
                  />
                </div>
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
