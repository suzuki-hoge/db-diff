import React, { type FC } from 'react'
import { ProjectInput } from '../../organisms/project-input/ProjectInput'
import styles from '../project-create/ProjectCreate.module.scss'
import { Header } from '../../molecules/header/Header'
import { type Project } from '../../../types'
import { IconBack } from '../../atoms/icon-back/IconBack'
import { useNavigate } from 'react-router-dom'

interface Props {
  insert: (project: Project) => void
}

export const ProjectCreate: FC<Props> = (props) => {
  const navigate = useNavigate()

  return (
    <div className={styles.template}>
      <Header
        globals={
          <IconBack
            variant={'large'}
            onClick={() => {
              navigate(-1)
            }}
          />
        }
        locals={<></>}
      />
      <ProjectInput save={props.insert} />
    </div>
  )
}
