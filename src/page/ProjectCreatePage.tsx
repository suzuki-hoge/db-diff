import { type FC } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { type Project } from '../types'
import { useNavigate } from 'react-router-dom'
import { ProjectCreate } from '../components/templates/project-create/ProjectCreate'

export const ProjectCreatePage: FC = () => {
  const navigate = useNavigate()

  const insert: (project: Project) => void = (project) => {
    console.log(project)
    invoke('insert_project_command', { projectJson: project })
      .then(() => {
        navigate('/project/list')
      })
      .catch(console.log)
  }

  return <ProjectCreate insert={insert} />
}
