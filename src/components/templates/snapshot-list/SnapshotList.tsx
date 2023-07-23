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
import { IconHelp } from '../../atoms/icon-help/IconHelp'
import { TourWrapper } from '../../atoms/tour-wrapper/TourWrapper'
import { type ReactourStep } from 'reactour'

interface Props {
  snapshotSummaries: SnapshotSummary[]
  remove: (id: string) => void
}

export const SnapshotList: FC<Props> = (props) => {
  const [isTouring, setIsTouring] = useState(false)
  const [isSetting, setIsSetting] = useState(false)
  const [selectedSnapshotSummary, setSelectedSnapshotSummary] = useState<SnapshotSummary | null>(null)

  const navigate = useNavigate()

  return (
    <>
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
              <IconHelp
                variant={'large'}
                onClick={() => {
                  setIsSetting(true)
                  setIsTouring(true)
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
      <TourWrapper
        steps={steps(props.snapshotSummaries.length === 0)}
        isTouring={isTouring}
        onClose={() => {
          setIsTouring(false)
          setIsSetting(false)
        }}
      />
    </>
  )
}

const steps: (isEmpty: boolean) => ReactourStep[] = (isEmpty) => {
  return isEmpty
    ? [
        {
          selector: '.icon_plus',
          content: 'まずは接続中 DB のスナップショットを 2 つ作成しましょう',
        },
        {
          content: (
            <>
              <span>スナップショットを 2 つ選択して差分を表示しましょう</span>
              <br />
              <br />
              <span>初回のみ差分計算をするため時間がかかる場合があります</span>
            </>
          ),
        },
        {
          selector: '.icon_gear',
          content: '作成したスナップショットの名前を編集することもできます',
        },
      ]
    : [
        {
          selector: '.text_card',
          content: (
            <>
              <span>スナップショットを 2 つ選択して差分を表示しましょう</span>
              <br />
              <br />
              <span>初回のみ差分計算をするため時間がかかる場合があります</span>
            </>
          ),
        },
        {
          selector: '.icon_plus',
          content: '新たなスナップショットを作成しましょう',
        },
        {
          selector: '.icon_gear',
          content: '作成したスナップショットを編集することができます',
        },
        {
          selector: '.icon_edit',
          content: 'スナップショットを編集できます',
        },
        {
          selector: '.icon_delete',
          content: 'スナップショットを削除できます',
        },
      ]
}
