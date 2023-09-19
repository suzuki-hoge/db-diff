import React, { type FC, useState } from 'react'
import styles from './SnapshotInput.module.scss'
import { createSnapshotId, type DumpConfig, type SnapshotSummary } from '../../../types'
import { Button } from '../../atoms/button/Button'
import { LabeledInputText } from '../../molecules/labeled-input-text/LabeledInputText'
import { z } from 'zod'
import { DumpConfigInput } from '../dump-config-input/DumpConfigInput'
import { IconVExpand } from '../../atoms/icon-v-expand/IconVExpand'

interface Props {
  snapshotSummary?: SnapshotSummary
  dumpConfigs: DumpConfig[]
  dump?: (snapshotId: string, snapshotName: string, dumpConfigs: DumpConfig[]) => void
  update?: (snapshotSummary: SnapshotSummary) => void
}

export const SnapshotInput: FC<Props> = (props) => {
  const [snapshotName, setSnapshotName] = useState(props.snapshotSummary?.snapshotName ?? '')
  const [dumpConfigValues, setDumpConfigValues] = useState(props.dumpConfigs.map((dumpConfig) => dumpConfig.value))

  const v = z.object({
    snapshotName: z.string().min(1, { message: '入力してください' }),
  })
  const [errors, setErrors] = useState<{
    snapshotName?: string[]
  }>({})

  const [isExpanded, setIsExpanded] = useState(false)

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
      <div className={styles.configs}>
        <span>
          Dump Configs
          <IconVExpand
            variant={'medium'}
            expanded={isExpanded}
            onClick={() => {
              setIsExpanded(!isExpanded)
            }}
          />
        </span>
        {props.dumpConfigs.map((dumpConfig, i) => (
          <div key={dumpConfig.tableName} className={[styles.collapse, isExpanded ? '' : styles.hide].join(' ')}>
            <DumpConfigInput
              key={dumpConfig.tableName}
              tableName={dumpConfig.tableName}
              colNames={dumpConfig.colNames}
              value={dumpConfigValues[i]}
              input={props.snapshotSummary === undefined}
              onChange={(value) => {
                const vs = [...dumpConfigValues]
                vs[i] = value
                setDumpConfigValues(vs)
              }}
            />
          </div>
        ))}
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
                const snapshotId = createSnapshotId()
                const dumpConfigs: DumpConfig[] = props.dumpConfigs.map((dumpConfig, i) => ({
                  tableName: dumpConfig.tableName,
                  colNames: dumpConfig.colNames,
                  value: dumpConfigValues[i],
                }))
                props.dump(snapshotId, snapshotName, dumpConfigs)
              }
            }
          }
        }}
      />
    </div>
  )
}
