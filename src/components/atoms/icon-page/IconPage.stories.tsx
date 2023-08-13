import type { Meta, StoryObj } from '@storybook/react'

import { IconPage } from './IconPage'

const meta = {
  title: 'Atoms/IconPage',
  component: IconPage,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof IconPage>

export default meta
type Story = StoryObj<typeof meta>

export const Left: Story = {
  args: {
    variant: 'medium',
    vector: 'left',
    end: false,
    enabled: true,
  },
}

export const LeftEnd: Story = {
  args: {
    variant: 'medium',
    vector: 'left',
    end: true,
    enabled: false,
  },
}

export const Right: Story = {
  args: {
    variant: 'medium',
    vector: 'right',
    end: false,
    enabled: false,
  },
}

export const RightEnd: Story = {
  args: {
    variant: 'medium',
    vector: 'right',
    end: true,
    enabled: true,
  },
}
