import type { Meta, StoryObj } from '@storybook/react'

import { ProjectUpdate } from './ProjectUpdate'
import { withRouter } from 'storybook-addon-react-router-v6'

const meta = {
  title: 'Templates/ProjectUpdate',
  component: ProjectUpdate,
  tags: ['autodocs'],
  argTypes: {},
  decorators: [withRouter],
} satisfies Meta<typeof ProjectUpdate>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {
  args: {
    project: {
      projectId: '92B07638-8EBA-471D-BDC1-71685B21EFE4',
      name: 'My Laravel Project',
      color: '#c2e0c6',
      rdbms: 'MySQL',
      user: 'admin',
      password: 'admin-pw',
      host: 'localhost',
      port: '3306',
      schema: 'my-laravel-project',
    },
    update: console.log,
  },
}
