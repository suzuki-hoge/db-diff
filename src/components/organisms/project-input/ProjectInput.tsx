import React, { type FC, useState } from 'react'
import styles from './ProjectInput.module.scss'
import { createProjectId, type Project } from '../../../types'
import { InputText } from '../../atoms/input-text/InputText'
import { IconSave } from '../../atoms/icon-save/IconSave'

interface Props {
  project?: Project
  save: (project: Project) => void
}

export const ProjectInput: FC<Props> = (props) => {
  const [name, setName] = useState(props.project?.name ?? '')
  const [user, setUser] = useState(props.project?.user ?? '')
  const [password, setPassword] = useState(props.project?.password ?? '')
  const [host, setHost] = useState(props.project?.host ?? '')
  const [port, setPort] = useState(props.project?.port ?? '')
  const [schema, setSchema] = useState(props.project?.schema ?? '')

  return (
    <div className={styles.component}>
      <div className={styles.item}>
        <span>Name</span>
        <InputText
          value={name}
          length={33}
          onInput={(e) => {
            setName(e.target.value)
          }}
        />
      </div>
      <div className={styles.item}>
        <span>RDBMS</span>
        <InputText value="MySQL" length={5} onInput={(_) => {}} />
      </div>
      <div className={styles.item}>
        <div className={styles.cols}>
          <div className={styles.item}>
            <span>Username</span>
            <InputText
              value={user}
              length={15}
              onInput={(e) => {
                setUser(e.target.value)
              }}
            />
          </div>
          <div className={styles.item}>
            <span>Password</span>
            <InputText
              value={password}
              length={15}
              onInput={(e) => {
                setPassword(e.target.value)
              }}
            />
          </div>
        </div>
      </div>
      <div className={styles.item}>
        <div className={styles.cols}>
          <div className={styles.item}>
            <span>Host</span>
            <InputText
              value={host}
              length={25}
              onInput={(e) => {
                setHost(e.target.value)
              }}
            />
          </div>
          <div className={styles.item}>
            <span>Port</span>
            <InputText
              value={port}
              length={5}
              onInput={(e) => {
                setPort(e.target.value)
              }}
            />
          </div>
        </div>
      </div>
      <div className={styles.item}>
        <span>Database</span>
        <InputText
          value={schema}
          length={20}
          onInput={(e) => {
            setSchema(e.target.value)
          }}
        />
      </div>
      <IconSave
        variant={'large'}
        onClick={() => {
          const projectId = props.project?.projectId ?? createProjectId()
          props.save({
            projectId,
            name,
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
  )
}
