import React, { type FC, useState } from 'react'
import styles from './DumpConfigInput.module.scss'
import { LabeledRadioText } from '../../molecules/labeled-radio-text/LabeledRadioText'
import { type DumpConfigValue } from '../../../types'
import { Select } from '../../atoms/select/Select'

interface Props {
  tableName: string
  colNames: string[]
  value: DumpConfigValue
  input: boolean
  onChange: (value: string) => void
}

function isOrdered(value: 'limited' | 'ignore' | string): boolean {
  return value !== 'limited' && value !== 'ignore'
}

export const DumpConfigInput: FC<Props> = (props) => {
  const [selectedColName, setSelectedColName] = useState(isOrdered(props.value) ? props.value : props.colNames[0])

  return (
    <div className={styles.component}>
      <LabeledRadioText
        label={props.tableName}
        value={isOrdered(props.value) ? 'ordered' : props.value}
        values={['ordered', 'ignore', 'limited']}
        displayValues={['Ordered 1,000', 'Ignore', 'Limited 1,000']}
        name={props.tableName}
        disabled={!props.input}
        onChange={(v) => {
          isOrdered(v) ? props.onChange(selectedColName) : props.onChange(v)
        }}
      />
      {isOrdered(props.value) && (
        <div className={styles.text}>
          <span>order by</span>
          <Select
            defaultValue={selectedColName}
            values={props.colNames}
            onChange={(v) => {
              setSelectedColName(v)
              props.onChange(v)
            }}
          />
          <span>desc</span>
        </div>
      )}
    </div>
  )
}
