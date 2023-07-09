import { type FC, useState } from 'react'
import styles from './SnapshotInput.module.scss'
import { type SnapshotSummary } from '../../../types'
import { InputText } from '../../atoms/input-text/InputText'

interface Props {
  snapshotSummary?: SnapshotSummary
}

export const SnapshotInput: FC<Props> = (props) => {
  const [name, setName] = useState(props.snapshotSummary?.snapshotName ?? '')

  return (
    <div className={styles.component}>
      <div className={styles.item}>
        <span>Name</span>
        <InputText
          value={name}
          length={20}
          onInput={(e) => {
            setName(e.target.value)
          }}
        />
      </div>
    </div>
  )
}
