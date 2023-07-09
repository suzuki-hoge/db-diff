import type { Meta, StoryObj } from '@storybook/react'

import { ProjectCreate } from './ProjectCreate'
import { withRouter } from 'storybook-addon-react-router-v6'

const meta = {
  title: 'Templates/ProjectCreate',
  component: ProjectCreate,
  tags: ['autodocs'],
  argTypes: {},
  decorators: [withRouter],
} satisfies Meta<typeof ProjectCreate>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {
  args: {
    insert: console.log,
  },
}
