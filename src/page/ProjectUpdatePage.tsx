import { type FC } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { type Project } from '../types'
import { useLocation, useNavigate } from 'react-router-dom'
import { ProjectUpdate } from '../components/templates/project-update/ProjectUpdate'
import { toast } from 'react-hot-toast'

export const ProjectUpdatePage: FC = () => {
  const location = useLocation()
  const navigate = useNavigate()

  const project = location.state as Project

  const update: (project: Project) => void = (project) => {
    toast.dismiss()
    invoke('update_project_command', { projectJson: project })
      .then(() => {
        toast.success('保存しました')
        navigate('/project/list')
      })
      .catch((e: string) => {
        navigate('/error', { state: { message: e } })
      })
  }

  const testConnection: (project: Project) => void = (project) => {
    toast.dismiss()
    invoke<string>('test_connection_project_command', { projectJson: project })
      .then((data) => {
        toast.success(data, { duration: 4000, style: { maxWidth: 600 } })
      })
      .catch((e: string) => {
        toast.error(e, { duration: 8000, style: { maxWidth: 600 } })
      })
  }

  return project !== undefined ? <ProjectUpdate project={project} update={update} testConnection={testConnection} /> : <></>
}
