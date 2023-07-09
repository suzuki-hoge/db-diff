import { type FC } from 'react'
import './global.scss'
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom'
import { ProjectListPage } from './page/ProjectListPage'
import { ProjectCreatePage } from './page/ProjectCreatePage'
import { ProjectUpdatePage } from './page/ProjectUpdatePage'

export const App: FC = () => {
  return (
    <div className="app">
      <Router>
        <Routes>
          <Route path="/" element={<ProjectListPage />} />
          <Route path="/project/list" element={<ProjectListPage />} />
          <Route path="/project/create" element={<ProjectCreatePage />} />
          <Route path="/project/update" element={<ProjectUpdatePage />} />
        </Routes>
      </Router>
    </div>
  )
}
