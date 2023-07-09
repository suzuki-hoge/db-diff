import { type FC, useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { type Project } from '../types'
import { useLocation, useNavigate } from 'react-router-dom'
import { ProjectList } from '../components/templates/project-list/ProjectList'

export const ProjectListPage: FC = () => {
  const [projects, setProjects] = useState<Project[]>([])

  const navigate = useNavigate()
  const location = useLocation()

  useEffect(() => {
    invoke<Project[]>('all_projects_command')
      .then((data) => {
        setProjects(data)
      })
      .catch(console.log)
  }, [location])

  const remove: (projectId: string) => void = (projectId) => {
    invoke('delete_project_command', { projectId })
      .then(() => {
        navigate('/project/list')
      })
      .catch(console.log)
  }

  return <ProjectList projects={projects} remove={remove} />
}
