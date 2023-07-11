import React, { type FC, useState } from 'react'
import styles from './SnapshotInput.module.scss'
import { type SnapshotSummary } from '../../../types'
import { InputText } from '../../atoms/input-text/InputText'
import { Button } from '../../atoms/button/Button'

interface Props {
  snapshotSummary?: SnapshotSummary
  dump?: (snapshotName: string) => void
  update?: (snapshotSummary: SnapshotSummary) => void
}

export const SnapshotInput: FC<Props> = (props) => {
  const [snapshotName, setSnapshotName] = useState(props.snapshotSummary?.snapshotName ?? '')

  return (
    <div className={styles.component}>
      <div className={styles.item}>
        <span>Name</span>
        <InputText
          value={snapshotName}
          length={20}
          onInput={(e) => {
            setSnapshotName(e.target.value)
          }}
        />
      </div>
      <Button
        variant={'primary'}
        label={'Save'}
        onClick={() => {
          if (props.snapshotSummary !== undefined) {
            if (props.update != null) {
              props.update({
                snapshotId: props.snapshotSummary.snapshotId,
                snapshotName,
                createAt: props.snapshotSummary.createAt,
              })
            }
          } else {
            if (props.dump != null) {
              props.dump(snapshotName)
            }
          }
        }}
      />
    </div>
  )
}
