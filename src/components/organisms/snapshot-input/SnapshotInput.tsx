import React, { type FC, useState } from 'react'
import styles from './SnapshotInput.module.scss'
import { type SnapshotSummary } from '../../../types'
import { Button } from '../../atoms/button/Button'
import { LabeledInputText } from '../../molecules/labeled-input-text/LabeledInputText'

interface Props {
  snapshotSummary?: SnapshotSummary
  dump?: (snapshotName: string) => void
  update?: (snapshotSummary: SnapshotSummary) => void
}

export const SnapshotInput: FC<Props> = (props) => {
  const [snapshotName, setSnapshotName] = useState(props.snapshotSummary?.snapshotName ?? '')

  return (
    <div className={styles.component}>
      <LabeledInputText label={'Name'} value={snapshotName} maxLength={20} onChange={setSnapshotName} />
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
