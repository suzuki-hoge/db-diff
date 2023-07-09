import { type FC, useState } from 'react'
import styles from './SnapshotList.module.scss'
import { TextCard } from '../../molecules/text-card/TextCard'
import { type SnapshotSummary } from '../../../types'
import { Header } from '../../molecules/header/Header'
import { IconBack } from '../../atoms/icon-back/IconBack'
import { ModalWindow } from '../../molecules/ModalWindow/ModalWindow'
import { SnapshotInput } from '../../organisms/snapshot-input/SnapshotInput'
import { IconPlus } from '../../atoms/icon-plus/IconPlus'
import { IconGear } from '../../atoms/icon-gear/IconGear'
import { IconEdit } from '../../atoms/icon-edit/IconEdit'
import { IconDelete } from '../../atoms/icon-delete/IconDelete'
import { IconSave } from '../../atoms/icon-save/IconSave'

interface Props {
  snapshotSummaries: SnapshotSummary[]
}

export const SnapshotList: FC<Props> = (props) => {
  const [isSetting, setIsSetting] = useState(false)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [snapshot, setSnapshot] = useState<SnapshotSummary | undefined>(
    undefined
  )
  const [selectedId, setSelectedId] = useState<string | null>(null)

  return (
    <div className={styles.template}>
      <Header
        globals={<IconBack variant={'large'} onClick={() => {}} />}
        locals={
          <>
            <IconPlus
              variant={'large'}
              onClick={() => {
                setSnapshot(undefined)
                setIsModalOpen(true)
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
                selected={selectedId === snapshotSummary.snapshotId}
                onClick={() => {
                  if (selectedId === null) {
                    setSelectedId(snapshotSummary.snapshotId)
                  } else if (selectedId === snapshotSummary.snapshotId) {
                    setSelectedId(null)
                  } else {
                    alert(2)
                  }
                }}
              />
              {isSetting && (
                <div className={styles.icons}>
                  <IconEdit
                    variant={'medium'}
                    onClick={() => {
                      setSnapshot(snapshotSummary)
                      setIsModalOpen(true)
                    }}
                  />
                  <IconDelete variant={'medium'} onClick={() => {}} />
                </div>
              )}
            </div>
          ))}
        </div>
      </div>
      <ModalWindow
        isOpen={isModalOpen}
        setIsOpen={setIsModalOpen}
        button={
          <IconSave
            variant={'large'}
            onClick={() => {
              console.log(42)
            }}
          />
        }
      >
        <SnapshotInput snapshotSummary={snapshot} />
      </ModalWindow>
    </div>
  )
}
