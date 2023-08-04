import React, { type FC, useState } from 'react'
import styles from './ProjectInput.module.scss'
import { createProjectId, type Project } from '../../../types'
import { Button } from '../../atoms/button/Button'
import { LabeledColorTagInput } from '../../molecules/labeled-color-tag-input/LabeledColorTagInput'
import { LabeledInputText } from '../../molecules/labeled-input-text/LabeledInputText'
import { LabeledRadioText } from '../../molecules/labeled-radio-text/LabeledRadioText'

interface Props {
  project?: Project
  save: (project: Project) => void
  testConnection: (project: Project) => void
}

export const ProjectInput: FC<Props> = (props) => {
  const [name, setName] = useState(props.project?.name ?? '')
  const [color, setColor] = useState(props.project?.color ?? '#c2e0c6')
  const [rdbms, setRdbms] = useState(props.project?.rdbms ?? 'MySQL')
  const [user, setUser] = useState(props.project?.user ?? '')
  const [password, setPassword] = useState(props.project?.password ?? '')
  const [host, setHost] = useState(props.project?.host ?? '')
  const [port, setPort] = useState(props.project?.port ?? '')
  const [schema, setSchema] = useState(props.project?.schema ?? '')

  return (
    <div className={styles.component}>
      <div className={styles.grid}>
        <LabeledColorTagInput label={'Name'} value={name} maxLength={30} onChange={setName} color={color} setColor={setColor} autoFocus={true} />
      </div>

      <LabeledRadioText label={'System'} value={rdbms} values={['MySQL', 'PostgreSQL']} name={'rdbms'} onChange={setRdbms} />

      <div className={styles.grid} style={{ gridTemplateColumns: '1fr 1rem 1fr' }}>
        <LabeledInputText value={user} label={'User'} maxLength={256} onChange={setUser} chars={'half'} />
        <div></div>
        <LabeledInputText value={password} label={'Password'} maxLength={256} onChange={setPassword} chars={'half'} />
      </div>

      <div className={styles.grid} style={{ gridTemplateColumns: '1fr 1rem 100px' }}>
        <LabeledInputText value={host} label={'Host'} maxLength={256} onChange={setHost} chars={'half'} />
        <div></div>
        <LabeledInputText value={port} label={'Port'} maxLength={6} onChange={setPort} chars={'number'} />
      </div>

      <div className={styles.grid}>
        <LabeledInputText value={schema} label={'Database'} maxLength={256} onChange={setSchema} chars={'half'} />
      </div>

      <div className={styles.buttons}>
        <Button
          variant={'primary'}
          label={'Save'}
          onClick={() => {
            const projectId = props.project?.projectId ?? createProjectId()
            props.save({
              projectId,
              name,
              color,
              rdbms: 'MySQL',
              user,
              password,
              host,
              port,
              schema,
            })
          }}
        />
        <Button
          variant={'secondary'}
          label={'Test Connection'}
          onClick={() => {
            props.testConnection({
              projectId: '',
              name,
              color,
              rdbms: 'MySQL',
              user,
              password,
              host,
              port,
              schema,
            })
          }}
        />
      </div>
    </div>
  )
}
