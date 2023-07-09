import { type FC } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { type Project } from '../types'
import { useLocation, useNavigate } from 'react-router-dom'
import { ProjectUpdate } from '../components/templates/project-update/ProjectUpdate'

export const ProjectUpdatePage: FC = () => {
  const location = useLocation()
  const navigate = useNavigate()

  const project = location.state as Project
  console.log(project)

  const update: (project: Project) => void = (project) => {
    invoke('update_project_command', { projectJson: project })
      .then(() => {
        navigate('/project/list')
      })
      .catch(console.log)
  }

  return project !== undefined ? (
    <ProjectUpdate project={project} update={update} />
  ) : (
    <></>
  )
}
