import React, { type FC, useState } from 'react'
import styles from './SnapshotInput.module.scss'
import { type SnapshotSummary } from '../../../types'
import { Button } from '../../atoms/button/Button'
import { LabeledInputText } from '../../molecules/labeled-input-text/LabeledInputText'
import { z } from 'zod'

interface Props {
  snapshotSummary?: SnapshotSummary
  dump?: (snapshotName: string) => void
  update?: (snapshotSummary: SnapshotSummary) => void
}

export const SnapshotInput: FC<Props> = (props) => {
  const [snapshotName, setSnapshotName] = useState(props.snapshotSummary?.snapshotName ?? '')

  const v = z.object({
    snapshotName: z.string().min(1, { message: '入力してください' }),
  })
  const [errors, setErrors] = useState<{
    snapshotName?: string[]
  }>({})

  return (
    <div className={styles.component}>
      <div className={styles.grid}>
        <LabeledInputText
          label={'Name'}
          value={snapshotName}
          maxLength={30}
          onChange={setSnapshotName}
          chars={'all'}
          autoFocus={true}
          errors={errors.snapshotName}
        />
      </div>
      <Button
        variant={'primary'}
        label={'Save'}
        onClick={() => {
          const r = v.safeParse({ snapshotName })

          if (!r.success) {
            setErrors(r.error.flatten().fieldErrors)
          } else {
            setErrors({})

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
          }
        }}
      />
    </div>
  )
}
