import type { Meta, StoryObj } from '@storybook/react'

import { Error } from './Error'
import { withRouter } from 'storybook-addon-react-router-v6'

const meta = {
  title: 'Templates/Error',
  component: Error,
  tags: ['autodocs'],
  argTypes: {},
  decorators: [withRouter],
} satisfies Meta<typeof Error>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {
  args: {
    message:
      "DriverError { Could not connect to address '127.0.0.1:20000': Connection refused (os error 61) } DriverError { Could not connect to address '127.0.0.1:20000': Connection refused (os error 61) } DriverError { Could not connect to address '127.0.0.1:20000': Connection refused (os error 61) }",
  },
}
