import React, { type FC } from 'react'
import styles from './SnapshotCreate.module.scss'
import { useNavigate } from 'react-router-dom'
import { Header } from '../../molecules/header/Header'
import { IconBack } from '../../atoms/icon-back/IconBack'
import { SnapshotInput } from '../../organisms/snapshot-input/SnapshotInput'
import { type DumpConfig } from '../../../types'

interface Props {
  dumpConfigs: DumpConfig[]
  dump: (snapshotName: string) => void
}

export const SnapshotCreate: FC<Props> = (props) => {
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
        <SnapshotInput dumpConfigs={props.dumpConfigs} dump={props.dump} />
      </div>
    </div>
  )
}
