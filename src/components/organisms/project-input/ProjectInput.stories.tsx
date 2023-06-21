import type { Meta, StoryObj } from '@storybook/react'

import { ProjectInput } from './ProjectInput'

const meta = {
  title: 'Organisms/ProjectInput',
  component: ProjectInput,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof ProjectInput>

export default meta
type Story = StoryObj<typeof meta>

export const Create: Story = {
  args: {},
}

export const Update: Story = {
  args: {
    project: {
      id: '92B07638-8EBA-471D-BDC1-71685B21EFE4',
      name: 'My Laravel Project',
      color: 'yellow',
      user: 'admin',
      password: 'admin-pw',
      host: 'localhost',
      port: '3306',
      schema: 'my-laravel-project',
      rdbms: 'MySQL',
    },
  },
}
