import { type FC, useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { type Project } from '../types'
import { useLocation, useNavigate } from 'react-router-dom'
import { ProjectList } from '../components/templates/project-list/ProjectList'
import { toast } from 'react-hot-toast'

export const ProjectListPage: FC = () => {
  const [projects, setProjects] = useState<Project[]>([])

  const navigate = useNavigate()
  const location = useLocation()

  useEffect(() => {
    invoke<Project[]>('all_projects_command')
      .then((data) => {
        setProjects(data)
      })
      .catch((e: string) => {
        navigate('/error', { state: { message: e } })
      })
  }, [location])

  const select: (projectId: string) => void = (projectId) => {
    invoke('select_project_command', { projectId })
      .then(() => {
        toast.success('DB に接続しました')
        navigate('/snapshot-summary/list')
      })
      .catch(() => {
        toast.error('DB 接続エラー： 設定を見直してください')
      })
  }

  const remove: (projectId: string) => void = (projectId) => {
    invoke('delete_project_command', { projectId })
      .then(() => {
        toast.success('削除しました')
        navigate('/project/list')
      })
      .catch((e: string) => {
        navigate('/error', { state: { message: e } })
      })
  }

  return <ProjectList projects={projects} select={select} remove={remove} />
}
