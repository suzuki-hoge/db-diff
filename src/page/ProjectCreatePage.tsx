import { type FC } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { type Project } from '../types'
import { useNavigate } from 'react-router-dom'
import { ProjectCreate } from '../components/templates/project-create/ProjectCreate'
import { toast } from 'react-hot-toast'

export const ProjectCreatePage: FC = () => {
  const navigate = useNavigate()

  const insert: (project: Project) => void = (project) => {
    toast.dismiss()
    invoke('insert_project_command', { projectJson: project })
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
        toast.success(data, { duration: 4000, style: { minWidth: 900 } })
      })
      .catch((e: string) => {
        toast.error(e)
      })
  }

  return <ProjectCreate insert={insert} testConnection={testConnection} />
}
