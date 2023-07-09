import { type FC, useState } from 'react'
import styles from './ProjectList.module.scss'
import { type Project } from '../../../types'
import { ColorTagCard } from '../../molecules/color-tag-card/ColorTagCard'
import { Header } from '../../molecules/header/Header'
import { IconPlus } from '../../atoms/icon-plus/IconPlus'
import { IconGear } from '../../atoms/icon-gear/IconGear'
import { IconEdit } from '../../atoms/icon-edit/IconEdit'
import { IconDelete } from '../../atoms/icon-delete/IconDelete'
import { useNavigate } from 'react-router-dom'

interface Props {
  projects: Project[]
  remove: (id: string) => void
}

export const ProjectList: FC<Props> = (props) => {
  const [isSetting, setIsSetting] = useState(false)

  const navigate = useNavigate()

  return (
    <div className={styles.template}>
      <Header
        globals={<></>}
        locals={
          <>
            <IconPlus
              variant={'large'}
              onClick={() => {
                navigate('/project/create')
              }}
            />
            <IconGear
              variant={'large'}
              onClick={() => {
                setIsSetting(!isSetting)
              }}
            />
          </>
        }
      />
      <div className={styles.component}>
        {props.projects.map((project, i) => (
          <div key={i} className={styles.item}>
            <ColorTagCard
              label={project.name}
              variant="green"
              onClick={() => {
                console.log(project.projectId)
              }}
            />
            {isSetting && (
              <div className={styles.icons}>
                <IconEdit
                  variant={'medium'}
                  onClick={() => {
                    navigate('/project/update', { state: project })
                  }}
                />
                <IconDelete
                  variant={'medium'}
                  onClick={() => {
                    props.remove(project.projectId)
                  }}
                />
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  )
}
