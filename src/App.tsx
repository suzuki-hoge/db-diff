import React, { type FC } from 'react'
import './global.scss'
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom'
import { ProjectListPage } from './page/ProjectListPage'
import { ProjectCreatePage } from './page/ProjectCreatePage'
import { ProjectUpdatePage } from './page/ProjectUpdatePage'
import { SnapshotCreatePage } from './page/SnapshotCreatePage'
import { SnapshotListPage } from './page/SnapshotListPage'
import { SnapshotUpdatePage } from './page/SnapshotUpdatePage'
import { DiffPage } from './page/DiffPage'

export const App: FC = () => {
  return (
    <div className="app">
      <Router>
        <Routes>
          <Route path="/" element={<ProjectListPage />} />
          <Route path="/project/list" element={<ProjectListPage />} />
          <Route path="/project/create" element={<ProjectCreatePage />} />
          <Route path="/project/update" element={<ProjectUpdatePage />} />
          <Route path="/snapshot-summary/list" element={<SnapshotListPage />} />
          <Route path="/snapshot-summary/create" element={<SnapshotCreatePage />} />
          <Route path="/snapshot-summary/update" element={<SnapshotUpdatePage />} />
          <Route path="/diff" element={<DiffPage />} />
        </Routes>
      </Router>
    </div>
  )
}
