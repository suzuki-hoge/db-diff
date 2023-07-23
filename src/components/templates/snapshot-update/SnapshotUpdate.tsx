import React, { type FC } from 'react'
import styles from './SnapshotUpdate.module.scss'
import { type SnapshotSummary } from '../../../types'
import { useNavigate } from 'react-router-dom'
import { Header } from '../../molecules/header/Header'
import { IconBack } from '../../atoms/icon-back/IconBack'
import { SnapshotInput } from '../../organisms/snapshot-input/SnapshotInput'

interface Props {
  snapshotSummary: SnapshotSummary
  update: (snapshotSummary: SnapshotSummary) => void
}

export const SnapshotUpdate: FC<Props> = (props) => {
  const navigate = useNavigate()

  return (
    <div className={styles.template}>
      <Header
        globals={
          <IconBack
            variant={'large'}
            onClick={() => {
              navigate('/snapshot-summary/list')
            }}
          />
        }
        locals={<></>}
      />
      <div className={styles.component}>
        <SnapshotInput snapshotSummary={props.snapshotSummary} update={props.update} />
      </div>
    </div>
  )
}
